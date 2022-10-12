import Switch from '@mui/material/Switch';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import Paper from '@mui/material/Paper';
import Stack from '@mui/material/Stack';

interface StatusInputs {
    onPowerToggle: () => void,
    power: boolean,
    audioInfo: string,
}

const StatusCard = ({ onPowerToggle, power, audioInfo }: StatusInputs) => {
    return <Paper
        sx={{
            p: 2,
            display: 'flex',
            flexDirection: 'column',
            //height: 240,
        }}
    >
        <FormGroup>
            <Stack spacing={2} direction="row" sx={{ mb: 1 }} justifyContent="space-between" >
                <FormControlLabel control={<Switch
                    checked={power}
                    onChange={onPowerToggle}
                />
                } label="Power" />
                <div style={{
                    display: "flex",
                    //alignItems: "center",
                }}>
                    <p style={{
                        fontSize: "100%",
                        display: "block",
                        width: "100%",
                        textAlign: "right"
                    }}>{audioInfo}</p>
                </div>
            </Stack>
        </FormGroup>

    </Paper>
}

export default StatusCard