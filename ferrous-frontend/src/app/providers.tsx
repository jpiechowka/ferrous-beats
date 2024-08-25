import {MusicPlayerProvider} from '@/contexts/MusicPlayerContext'
import {NextUIProvider} from '@nextui-org/react'
import {ToolsProvider} from "@/contexts/ToolsContext";

export function Providers({children}: { children: React.ReactNode }) {
    return (
        <NextUIProvider>
            <ToolsProvider>
                <MusicPlayerProvider>
                    {children}
                </MusicPlayerProvider>
            </ToolsProvider>
        </NextUIProvider>
    )
}
