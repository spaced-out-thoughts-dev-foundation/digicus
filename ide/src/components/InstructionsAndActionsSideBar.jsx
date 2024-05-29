// import BarChart from './BarChart'
import React from 'react';
import { Button, List, ListItem } from '@mui/material';
import { supportedInstructionToColor } from '../common/InstructionNode';


import ".././styles/InstructionsAndActionsSideBar.css";

function InstructionsAndActionsSideBar({handleDeploy, supportedInstructions}) {
  return (
    <div className='instructions-and-actions-sidebar'>
      <div className='instructions-menu-header'>
        <h2>Instructions Menu</h2>
      </div>
        <div className='instructions-menu-list-container'>
        <List>
          {supportedInstructions?.map((supported_instruction_data, index) => (
            <ListItem key={index} className='instructions-menu-list-item' style={{backgroundColor: supportedInstructionToColor(supported_instruction_data)}}>
              <Button className='instructions-menu-list-item-button'>
                {<strong style={{color: 'black', fontSize: '0.9em', marginRight: '0.5em',  textShadow: '1px 1px 1px gray'}}>{supported_instruction_data.name}</strong>}{'(' + supported_instruction_data.category + ')'}
              </Button>
            </ListItem>
          ))}
        </List>

        </div>
      <div className='deploy-button-container'>
        <button style={{width: '100%'}} onClick={handleDeploy}>Deploy</button>
      </div>

      </div>
  );
}
export default InstructionsAndActionsSideBar;