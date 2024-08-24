pub mod convert {
    pub mod audio;
}

pub mod download {
    pub mod audio;
}

pub mod identify {
    pub mod audio;
}

pub mod shared {
    pub mod functions {
        pub mod commands;
        pub mod files;
        pub mod tools;
    }

    pub mod model {
        pub mod acoustid;
        pub mod commands;
        pub mod media;
        pub mod musicbrainz;
        pub mod tools;
    }
}

pub mod library {
    pub mod list;
    pub mod play;
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
