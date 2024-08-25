import {MusicPlayerProvider} from '@/contexts/MusicPlayerContext'
import {NextUIProvider} from '@nextui-org/react'

export function Providers({children}: { children: React.ReactNode }) {
    return (
        <NextUIProvider>
            <MusicPlayerProvider>
                {children}
            </MusicPlayerProvider>
        </NextUIProvider>
    )
}
