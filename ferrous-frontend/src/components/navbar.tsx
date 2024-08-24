"use client";

import {Button} from "@nextui-org/button";
import {
    Navbar,
    NavbarBrand,
    NavbarContent,
    NavbarItem,
    NavbarMenu,
    NavbarMenuItem,
    NavbarMenuToggle
} from "@nextui-org/navbar";
import {Link} from "@nextui-org/link";
import React, {useState} from "react";
import {FerrousBeatsLogo} from "@/components/logo";
import {Tooltip} from "@nextui-org/tooltip";

export default function FerrousNavbar() {
    const [isMenuOpen, setIsMenuOpen] = useState(false);

    const githubMainRepoUrl: string = "https://github.com/jpiechowka/ferrous-beats";
    const tooltipDelay: number = 500;

    const navbarItems = new Map<string, {
        color: "foreground" | "primary" | "secondary" | "success" | "warning" | "danger",
        href: string,
        tooltipContent: string,
    }>([
        ["Library", {color: "primary", href: "#", tooltipContent: "View, tag and play files from local music library"}],
        ["Downloader", {
            color: "foreground",
            href: "#",
            tooltipContent: "Download music from different places using yt-dlp"
        }],
        ["Identifier", {
            color: "foreground",
            href: "#",
            tooltipContent: "Identify music files with Chromparint, AcoustID and MusicBrainz"
        }],
        ["Converter", {
            color: "foreground",
            href: "#",
            tooltipContent: "Convert music files to different formats using ffmpeg"
        }],
        ["Tools", {color: "foreground", href: "#", tooltipContent: "Download, update and manage various tools"}],
    ]);

    const menuItems = new Map<string, {
        color: "foreground" | "primary" | "secondary" | "success" | "warning" | "danger",
        href: string,
        target?: string,
        rel?: string
    }>([
        ["Music Library", {color: "primary", href: "#"}],
        ["Music Downloader (yt-dlp)", {color: "foreground", href: "#"}],
        ["Music Identifier", {color: "foreground", href: "#"}],
        ["Music Converter (ffmpeg)", {color: "foreground", href: "#"}],
        ["Tools Management", {color: "foreground", href: "#"}],
        ["Source Code", {color: "secondary", href: githubMainRepoUrl, target: "_blank", rel: "noopener noreferrer"}]
    ]);

    return (
        <Navbar onMenuOpenChange={setIsMenuOpen} isBordered maxWidth={"2xl"}>
            <NavbarContent>
                <NavbarMenuToggle
                    aria-label={isMenuOpen ? "Close menu" : "Open menu"}
                    className="md:hidden"
                />
                <NavbarBrand>
                    <FerrousBeatsLogo/>
                    <div className="w-2"></div>
                    <p className="font-bold text-inherit">Ferrous Beats</p>
                </NavbarBrand>
            </NavbarContent>

            <NavbarContent className="hidden md:flex gap-4" justify="center">
                {Array.from(navbarItems).map(([item, props], index) => (
                    <NavbarItem key={`${item}-${index}`}>
                        <Tooltip
                            key={`tooltip-${item}-${index}`}
                            content={props.tooltipContent}
                            showArrow
                            placement="bottom"
                            color={props.color}
                            delay={tooltipDelay}
                        >
                            <Link
                                key={`navbar-link-${item}-${index}`}
                                color={props.color}
                                href={props.href}
                                aria-current={props.color === "primary" ? "page" : undefined}
                            >
                                {item}
                            </Link>
                        </Tooltip>
                    </NavbarItem>
                ))}
            </NavbarContent>

            <NavbarContent justify="end">
                <NavbarItem className="hidden lg:flex">
                    <Tooltip key="source-code-tooltip" content="View source code on GitHub" showArrow placement="bottom"
                             color="secondary" delay={tooltipDelay}>
                        <Button as={Link} color="secondary" href={githubMainRepoUrl} target="_blank"
                                rel="noopener noreferrer" variant="ghost" radius="md">
                            Source Code
                        </Button>
                    </Tooltip>
                </NavbarItem>
                <NavbarItem>
                    <Tooltip key="report-bug-tooltip" content="Report a bug o on GitHub" showArrow placement="bottom"
                             color="danger" delay={tooltipDelay}>
                        <Button as={Link} color="danger" href={githubMainRepoUrl + "/issues"} target="_blank"
                                rel="noopener noreferrer" variant="ghost" radius="md">
                            Report a Bug
                        </Button>
                    </Tooltip>
                </NavbarItem>
            </NavbarContent>

            <NavbarMenu>
                {Array.from(menuItems).map(([item, props], index) => (
                    <NavbarMenuItem key={`${item}-${index}`}>
                        <Link
                            key={`menu-link-${item}-${index}`}
                            color={props.color}
                            className="w-full"
                            href={props.href}
                            size="lg"
                            {...(props.target && {target: props.target})}
                            {...(props.rel && {rel: props.rel})}
                        >
                            {item}
                        </Link>
                    </NavbarMenuItem>
                ))}
            </NavbarMenu>
        </Navbar>
    );
}
