pub mod convert {
    pub mod audio;
}

pub mod download {
    pub mod audio;
    pub mod video;
}

pub mod ffmpeg {
    pub mod download;
    pub mod status;
}

pub mod yt_dlp {
    pub mod download;
    pub mod status;
    pub mod update;
}

pub mod errors;
pub mod index;
pub mod shared_funcs;
pub mod shared_model;
