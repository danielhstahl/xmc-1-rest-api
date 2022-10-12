import Stack from '@mui/material/Stack';
import Slider from '@mui/material/Slider';
import VolumeDown from '@mui/icons-material/VolumeDown';
import VolumeUp from '@mui/icons-material/VolumeUp';
import IconButton from '@mui/material/IconButton';
import Paper from '@mui/material/Paper';

interface VolumeInputs {
    onVolumeChange: (v: number) => void,
    volumeUp: () => Promise<Response>,
    volumeDown: () => Promise<Response>,
    volume: number,

}
const VolumeCard = ({ onVolumeChange, volumeUp, volumeDown, volume }: VolumeInputs) => {
    return <Paper
        sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
            height: 240,
        }}
    >
        <Stack spacing={2} direction="row" sx={{ mb: 1 }} alignItems="center">
            <IconButton onClick={volumeDown} ><VolumeDown /></IconButton>
            <Slider min={-66} max={11} aria-label="Volume" value={volume} onChange={(_e: Event, n: number | number[]) => {
                const volume = n as number
                onVolumeChange(volume)
            }} />
            <IconButton onClick={volumeUp} > <VolumeUp /></IconButton>
        </Stack>
        <Stack spacing={2} direction="row" sx={{ mb: 1 }} alignItems="center" style={{
            display: "flex",
            alignItems: "center",


        }}>

            <p style={{
                fontSize: "300%",
                display: "block",
                width: "100%",
                textAlign: "center"
            }}>{volume}</p>
        </Stack>

    </Paper>
}

export default VolumeCard