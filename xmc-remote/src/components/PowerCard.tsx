import Switch from '@mui/material/Switch';
import FormControlLabel from '@mui/material/FormControlLabel';
import Paper from '@mui/material/Paper';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import { Source, Mode } from '../services/api'
import Grid from '@mui/system/Unstable_Grid'
import Select, { SelectChangeEvent } from '@mui/material/Select';


interface PowerInputs {
    onPowerToggle: () => void,
    power: boolean,
    onInputChange: (e: SelectChangeEvent) => void,
    source: Source,
    mode: Mode,
    onModeChange: (e: SelectChangeEvent) => void

}
const SOURCE = "Source"
const MODE = "Mode"

//const MODE_OPTIONS = [{ label: "Stereo", }]

const PowerCard = ({ onPowerToggle, power, onInputChange, source, mode, onModeChange }: PowerInputs) => {
    return <Paper
        sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
        }}
    >
        <Grid container spacing={2}>
            <Grid xs={6} style={{
                display: "flex",
            }}>
                <FormControlLabel
                    control={<Switch checked={power} onChange={onPowerToggle} />}
                    label="Power"
                />
            </Grid>
            <Grid xs={6}>
                <FormControl size="small" fullWidth >
                    <InputLabel id="source-select-label">{SOURCE}</InputLabel>
                    <Select
                        labelId="source-select-label"
                        id="source-select"
                        value={source}
                        label={SOURCE}
                        onChange={onInputChange}
                    >
                        {Object.values(Source).map((v) => <MenuItem key={v} value={v}>{v}</MenuItem>)}
                    </Select>
                </FormControl>
            </Grid>
            <Grid xs={6}></Grid>
            <Grid xs={6}>
                <FormControl size="small" fullWidth >
                    <InputLabel id="source-select-label">{MODE}</InputLabel>
                    <Select
                        labelId="source-select-label"
                        id="source-select"
                        value={mode}
                        label={MODE}
                        onChange={onModeChange}
                    >
                        {Object.values(Mode).map((v) => <MenuItem key={v} value={v}>{v}</MenuItem>)}
                    </Select>
                </FormControl>
            </Grid>
        </Grid>
    </Paper >
}

export default PowerCard