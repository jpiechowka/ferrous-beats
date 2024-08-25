"use client";

import FerrousNavbar from "@/components/FerrousNavbar";
import React from "react";
import MusicLibraryTable from "@/components/MusicLibraryTable";

export default function LibraryPage() {
    return (
        <>
            <FerrousNavbar></FerrousNavbar>
            <MusicLibraryTable></MusicLibraryTable>
        </>
    );
}