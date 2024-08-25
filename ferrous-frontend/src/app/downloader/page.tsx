import React from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";

export default function DownloaderPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <div>
                <h1>Download Music with yt-dlp</h1>
            </div>
            <MusicPlayer></MusicPlayer>
        </>
    );
}
