// import BarChart from './BarChart'
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Avatar from '@mui/material/Avatar';
import InfoHeader from './InfoHeader'
import React from 'react';

import ".././styles/App.css";

function TopHeaderBar() {
  return (
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
      <Typography style={{fontSize: '3em'}}>
        Digicus IDE
      </Typography>
    </div>
    <div style={{flex: 1, justifyContent: 'right', display: 'flex'}}>
      <InfoHeader />
    </div>
  
    </Toolbar>
  )
}
export default TopHeaderBar;