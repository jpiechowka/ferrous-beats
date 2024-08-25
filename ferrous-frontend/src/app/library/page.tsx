"use client";

import FerrousNavbar from "@/components/FerrousNavbar";
import React from "react";
import MusicLibraryTable from "@/components/MusicLibraryTable";
import MusicPlayer from "@/components/MusicPlayer";
import {Divider} from "@nextui-org/divider";

export default function LibraryPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <MusicLibraryTable></MusicLibraryTable>
            <Divider className="my-4"/>
            <MusicPlayer></MusicPlayer>
        </>
    );
}