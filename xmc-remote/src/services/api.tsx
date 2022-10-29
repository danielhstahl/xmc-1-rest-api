export enum Power {
    On = "On",
    Off = "Off",
}
//why am I geting "surround" back as a mode?
/*export enum Mode {
    stereo = "Stereo",
    dolby = "Dolby",
    dts = "DTS",
    allstereo = "All Stereo",
    auto = "Auto",
    referencestereo = "Reference Stereo",
    surround = "Surround" //figure what this actually means!
}*/

export enum Source {
    HDMI1 = "HDMI 1",
    HDMI2 = "HDMI 2",
    HDMI3 = "HDMI 3",
    HDMI4 = "HDMI 4",
    HDMI5 = "HDMI 5",
    HDMI6 = "HDMI 6",
    HDMI7 = "HDMI 7",
    HDMI8 = "HDMI 8"
}

export interface XmcReadOnly {
    mode: string;
    audioBits: string,
    audioBitstream: string
    videoFormat: string
};

export interface XmcWrite {
    power: Power;
    source: Source;
    volume: number;
}



export const getReadOnly = () => fetch("/info").then(res => res.json()).then(r => {
    const { mode, audioBits, audioBitstream, videoFormat } = r
    return { mode, audioBits, audioBitstream, videoFormat } as XmcReadOnly
})
export const getWrite = () => fetch("/info").then(res => res.json()).then(r => {
    const { power, source, volume } = r
    return { power, source, volume } as XmcWrite
})
export const volumeUp = () => fetch("/volume/up", { method: "POST" })
export const volumeDown = () => fetch("/volume/down", { method: "POST" })
export const setVolume = (volume: number) => fetch(`/volume/${volume}`, { method: "POST" })
export const setSource = (source: Source) => fetch(`/input/${source}`, { method: "POST" })
export const powerOn = () => fetch(`/power/on`, { method: "POST" })
export const standBy = () => fetch(`/power/off`, { method: "POST" })