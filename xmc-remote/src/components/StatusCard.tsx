import Switch from '@mui/material/Switch';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import Paper from '@mui/material/Paper';
import Stack from '@mui/material/Stack';
import { Source, Mode, } from '../services/api'

interface StatusInputs {
    onPowerToggle: () => void,
    power: boolean,
    onModeChange: (e: SelectChangeEvent) => void,
    mode: Mode,
    onInputChange: (e: SelectChangeEvent) => void,
    source: Source
}
const AUDIO_MODE = "Audio Mode"
const SOURCE = "Source"
const StatusCard = ({ onPowerToggle, power, onModeChange, mode, onInputChange, source }: StatusInputs) => {
    return <Paper
        sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
            height: 240,
        }}
    >
        <FormGroup>
            <Stack spacing={3} direction="column" sx={{ mb: 1 }} >
                <FormControlLabel control={<Switch
                    checked={power}
                    onChange={onPowerToggle}
                />
                } label="Power" />
                <FormControl fullWidth >
                    <InputLabel id="mode-select-select-label">{AUDIO_MODE}</InputLabel>
                    <Select
                        labelId="mode-select-select-label"
                        id="model-select"
                        value={mode}
                        label={AUDIO_MODE}
                        onChange={onModeChange}
                    >
                        {Object.values(Mode).map((v, i) => <MenuItem value={v}>{v}</MenuItem>)}
                    </Select>
                </FormControl>
                <FormControl fullWidth >
                    <InputLabel id="source-select-label">{SOURCE}</InputLabel>
                    <Select
                        labelId="source-select-label"
                        id="source-select"
                        value={source}
                        label={SOURCE}
                        onChange={onInputChange}
                    >
                        {Object.values(Source).map((v) => <MenuItem value={v}>{v}</MenuItem>)}
                    </Select>
                </FormControl>
            </Stack>
        </FormGroup>

    </Paper>
}

export default StatusCard