"use client";

import React from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";

export default function ConverterPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <div>
                <h1>Convert Music with ffmpeg</h1>
            </div>
            <MusicPlayer></MusicPlayer>
        </>
    );
}
