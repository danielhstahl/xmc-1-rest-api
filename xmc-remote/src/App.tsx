import { useEffect, useState } from 'react';
import './App.css';
import MuiAppBar, { AppBarProps as MuiAppBarProps } from '@mui/material/AppBar';
import { styled, createTheme, ThemeProvider } from '@mui/material/styles';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import Toolbar from '@mui/material/Toolbar';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import Switch from '@mui/material/Switch';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import { Source, Power, Mode, XmcStatus, getStatus, volumeUp, volumeDown, setVolume, setMode, setSource, powerOn, standBy } from './services/api'
import Stack from '@mui/material/Stack';
import Slider from '@mui/material/Slider';
import VolumeDown from '@mui/icons-material/VolumeDown';
import VolumeUp from '@mui/icons-material/VolumeUp';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

const drawerWidth: number = 240;
interface AppBarProps extends MuiAppBarProps {
  open?: boolean;
}
const mdTheme = createTheme();

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== 'open',
})<AppBarProps>(({ theme, open }) => ({
  zIndex: theme.zIndex.drawer + 1,
  transition: theme.transitions.create(['width', 'margin'], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}));

const AppBarMenu = () => {
  return <AppBar position="absolute" open={false}>
    <Toolbar
      sx={{
        pr: '24px', // keep right padding when drawer closed
      }}
    >
      <Typography
        component="h1"
        variant="h6"
        color="inherit"
        noWrap
        sx={{ flexGrow: 1 }}
      >
        XMC Remote
      </Typography>
    </Toolbar>
  </AppBar>
}
interface StatusInputs {
  onPowerToggle: () => void,
  power: boolean,
  onModeChange: (e: SelectChangeEvent) => void,
  mode: Mode,
  onInputChange: (e: SelectChangeEvent) => void,
  source: Source

}
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
      <FormControlLabel control={<Switch
        checked={power}
        onChange={onPowerToggle}
      />
      } label="Power" />
      <Space />
      <FormControl fullWidth >
        <InputLabel id="demo-simple-select-label">Mode</InputLabel>
        <Select
          labelId="demo-simple-select-label"
          id="demo-simple-select"
          value={mode}
          label="Mode"
          onChange={onModeChange}
        >
          {Object.values(Mode).map((v, i) => <MenuItem value={v}>{v}</MenuItem>)}
        </Select>
      </FormControl>
      <Space />
      <FormControl fullWidth >
        <InputLabel id="demo-simple-select-label">HDMI Input</InputLabel>
        <Select
          labelId="demo-simple-select-label"
          id="demo-simple-select"
          value={source}
          label="HDMI Input"
          onChange={onInputChange}
        >
          {Object.values(Source).map((v) => <MenuItem value={v}>{v}</MenuItem>)}
        </Select>
      </FormControl>
    </FormGroup>

  </Paper>
}

interface VolumeInputs {
  onVolumeChange: (v: number) => void,
  volume: number,

}
const VolumeCard = ({ onVolumeChange, volume }: VolumeInputs) => {
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
  </Paper>
}

const Space = () => <div style={{ height: 30 }}></div>

function App() {
  const [xmcStatus, setXmcStatus] = useState<XmcStatus>({ power: Power.Off, source: Source.HDMI1, volume: -100, mode: Mode.surround });
  useEffect(() => {
    setInterval(() => {
      getStatus().then(setXmcStatus)
    }, 2000)
  }, [])
  console.log(xmcStatus)
  const onPowerToggle = () => {
    //setXmcStatus(prev=>({...prev, power: prev.power==="on"?"off":"on"}))
    if (xmcStatus.power === Power.On) {
      setXmcStatus((prev: XmcStatus) => ({ ...prev, power: Power.Off }))
      standBy()
    }
    else if (xmcStatus.power === Power.Off) {
      console.log("setting status to something different")
      setXmcStatus((prev: XmcStatus) => ({ ...prev, power: Power.On }))
      powerOn()
    }
  }

  const onVolumeChange = (volume: number) => {

    setXmcStatus((prev: XmcStatus) => ({ ...prev, volume }))
    setVolume(volume)
  }

  const onModeChange = (e: SelectChangeEvent) => {
    const mode = e.target.value as Mode
    setXmcStatus((prev: XmcStatus) => ({ ...prev, mode }))
    setMode(mode)
  }

  const onInputChange = (e: SelectChangeEvent) => {
    const source = e.target.value as Source
    setXmcStatus((prev: XmcStatus) => ({ ...prev, source }))
    setSource(source)
  }

  return (
    <ThemeProvider theme={mdTheme}>
      <Box sx={{ display: 'flex' }}>
        <CssBaseline />
        <AppBarMenu />
        <Box
          component="main"
          sx={{
            backgroundColor: (theme) =>
              theme.palette.mode === 'light'
                ? theme.palette.grey[100]
                : theme.palette.grey[900],
            flexGrow: 1,
            height: '100vh',
            overflow: 'auto',
          }}
        >
          <Toolbar />
          <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
            <Grid container spacing={3}>
              <Grid item xs={12} md={8} lg={9}>
                <StatusCard
                  onPowerToggle={onPowerToggle}
                  power={xmcStatus.power === Power.On}
                  onModeChange={onModeChange}
                  mode={xmcStatus.mode}
                  onInputChange={onInputChange}
                  source={xmcStatus.source}
                />
              </Grid>
              <Grid item xs={12} md={4} lg={3}>
                <VolumeCard
                  onVolumeChange={onVolumeChange}
                  volume={xmcStatus.volume}
                />
              </Grid>
            </Grid>
          </Container>
        </Box>
      </Box>

    </ThemeProvider >
  );
}

export default App;
