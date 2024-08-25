"use client";

import React from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";

export default function IdentifierPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <div>
                <h1>Identify Music</h1>
            </div>
            <MusicPlayer></MusicPlayer>

        </>
    );
}
