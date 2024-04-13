// import BarChart from './BarChart'
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Avatar from '@mui/material/Avatar';
import { SpeedInsights } from "@vercel/speed-insights/react"

const App = () => {
    return ( 
      <div>
        <AppBar position="static">
          <Toolbar style={{ 
              display: 'flex', 
              backgroundColor: 'black',
            }}>
            <div style={{flex: 1}}>
              <Avatar alt="Logo" src="DevelopmentFoundation_copy_512x575.png" />
            </div>
            <div style={{
                flex: 1, 
                display: 'flex', 
                justifyContent: 'center',
                color: 'rgb(39 207 230)'
              }}>
              <Typography style={{fontSize: '1.5em'}}>
                Digicus IDE
              </Typography>
            </div>
            <div style={{flex: 1}}></div>
          </Toolbar>
          <SpeedInsights />
        </AppBar>
      </div>
    )
}
export default App;