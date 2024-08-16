pub mod convert {
    pub mod audio;
}

pub mod download {
    pub mod audio;
    pub mod video;
}

pub mod shared {
    pub mod functions {
        pub mod commands;
        pub mod files;
        pub mod tools;
    }

    pub mod model {
        pub mod commands;
        pub mod responses;
    }
}

pub mod tools {
    pub mod chromaprint {
        pub mod download;
        pub mod status;
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
}

pub mod errors;
pub mod index;
