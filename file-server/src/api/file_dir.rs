// @Author: westhide.yzw
// @Date: 2022-02-22 12:44:24
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:44:24

use std::path::PathBuf;

use async_recursion::async_recursion;
use chrono::{offset::Local, DateTime};
use futures::future;
use poem::{
    error::StaticFileError,
    handler,
    web::{Json, Query},
    Result,
};
use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};
use tokio::{fs, io};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Dir<T> {
    pub path: Option<T>,
    pub metadata: Metadata,
    pub children: Option<Vec<Dir<T>>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Metadata {
    pub file_type: Option<String>,
    pub size: u64,
    pub status: bool,
    pub message: Option<String>,
    created: Option<DateTime<Local>>,
    modified: Option<DateTime<Local>>,
    // accessed: Option<DateTime<Local>>,
}

impl Metadata {
    fn new(metadata: &std::fs::Metadata) -> Self {
        let mut file_type = None;
        if metadata.is_file() {
            file_type = Some("file".into())
        } else if metadata.is_dir() {
            file_type = Some("dir".into())
        } else if cfg!(windows) && metadata.is_symlink() {
            file_type = Some("symlink".into())
        }

        let created = match metadata.created() {
            Ok(created) => Some(created.into()),
            Err(_) => None,
        };
        let modified = match metadata.modified() {
            Ok(modified) => Some(modified.into()),
            Err(_) => None,
        };
        // let accessed = match metadata.accessed() {
        //     Ok(accessed) => Some(accessed.into()),
        //     Err(_) => None,
        // };

        Self {
            file_type,
            size: metadata.len(),
            status: true,
            created,
            modified,
            // accessed,
            ..Self::default()
        }
    }

    fn error(message: String) -> Self {
        Self {
            status: false,
            message: Some(message),
            ..Self::default()
        }
    }

    fn is_dir(&self) -> bool {
        self.file_type == Some("dir".into())
    }
}

impl Dir<String> {
    async fn new(dir_path: impl Into<PathBuf>, depth: isize) -> Result<Self, io::Error> {
        let dir_path_buf = dir_path.into();
        Self::make_dir(dir_path_buf, depth).await
    }

    #[async_recursion]
    async fn make_dir(path_buf: PathBuf, depth: isize) -> Result<Self, io::Error> {
        let mut dir = Dir::<std::string::String> {
            path: path_buf.to_str().map(str::to_string),
            ..Self::default()
        };

        let metadata = fs::metadata(&path_buf).await;
        match &metadata {
            Ok(metadata) => {
                dir.metadata = Metadata::new(metadata);
            }
            Err(err) => {
                dir.metadata = Metadata::error(format!("{}", err));
                return Ok(dir);
            }
        };

        if dir.metadata.is_dir() && depth != 0 {
            let mut async_queue = vec![];
            match path_buf.read_dir() {
                Ok(read_dir) => {
                    let child_depth = if depth > 0 { depth - 1 } else { -1 };
                    for entry in read_dir {
                        let children_path_buf = entry?.path();
                        let child = Self::make_dir(children_path_buf, child_depth);
                        async_queue.push(child);
                    }
                    let children = future::try_join_all(async_queue).await?;

                    dir.children = Some(children);
                }
                Err(err) => dir.metadata = Metadata::error(format!("{}", err)),
            }
        };

        Ok(dir)
    }

    async fn disk_dir(depth: isize) -> Result<Self, io::Error> {
        let mut dir = Self {
            path: Some('/'.into()),
            ..Self::default()
        };

        if depth != 0 {
            let mut system = System::new();
            system.refresh_disks_list();
            let disk_mount_points: Vec<_> = system
                .disks()
                .iter()
                .map(|disk| disk.mount_point())
                .collect();

            let mut async_queue = vec![];
            for disk_mount_point in disk_mount_points {
                let disk_path = disk_mount_point.to_str();
                if let Some(path) = disk_path {
                    let child = Self::new(path, depth - 1);
                    async_queue.push(child);
                }
            }
            let children = future::try_join_all(async_queue).await?;
            dir.children = Some(children);
        }

        Ok(dir)
    }
}

#[derive(Debug, Deserialize)]
pub struct FileDirParams {
    path: Option<String>,
    depth: Option<isize>,
}

#[handler]
pub async fn file_dir(
    Query(params): Query<FileDirParams>,
) -> Result<Json<Dir<String>>, StaticFileError> {
    let FileDirParams { path, depth } = params;

    let file_dir: Dir<String>;
    let depth = depth.unwrap_or(1);
    if path.is_none() {
        file_dir = Dir::disk_dir(depth).await?
    } else {
        file_dir = Dir::new(path.unwrap(), depth).await?
    }

    Ok(Json(file_dir))
}
