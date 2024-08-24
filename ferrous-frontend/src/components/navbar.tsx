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

export default function FerrousNavbar() {
    const [isMenuOpen, setIsMenuOpen] = useState(false);

    const githubMainRepoUrl = "https://github.com/jpiechowka/ferrous-beats";

    const navbarItems = new Map<string, {
        color: "foreground" | "primary" | "secondary" | "success" | "warning" | "danger",
        href: string
    }>([
        ["Library", {color: "primary", href: "#"}],
        ["Downloader", {color: "foreground", href: "#"}],
        ["Identifier", {color: "foreground", href: "#"}],
        ["Converter", {color: "foreground", href: "#"}],
        ["Tools", {color: "foreground", href: "#"}],
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
                        <Link
                            color={props.color}
                            href={props.href}
                            aria-current={props.color === "primary" ? "page" : undefined}
                        >
                            {item}
                        </Link>
                    </NavbarItem>
                ))}
            </NavbarContent>

            <NavbarContent justify="end">
                <NavbarItem className="hidden lg:flex">
                    <Button as={Link} color="secondary" href={githubMainRepoUrl} target="_blank"
                            rel="noopener noreferrer" variant="ghost" radius="md">
                        Source Code
                    </Button>
                </NavbarItem>
                <NavbarItem>
                    <Button as={Link} color="danger" href={githubMainRepoUrl + "/issues"} target="_blank"
                            rel="noopener noreferrer" variant="ghost" radius="md">
                        Report a Bug
                    </Button>
                </NavbarItem>
            </NavbarContent>

            <NavbarMenu>
                {Array.from(menuItems).map(([item, props], index) => (
                    <NavbarMenuItem key={`${item}-${index}`}>
                        <Link
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
