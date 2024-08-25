"use client";

import React from 'react';
import FerrousNavbar from "@/components/FerrousNavbar";
import MusicPlayer from "@/components/MusicPlayer";
import {Divider} from "@nextui-org/divider";

export default function IdentifierPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <div>
                <h1>Identify Music</h1>
            </div>

            <Divider className="my-4"/>

            <MusicPlayer></MusicPlayer>

        </>
    );
}
