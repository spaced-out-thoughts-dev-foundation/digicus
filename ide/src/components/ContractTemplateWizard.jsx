import React, { useEffect, useState } from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import ListItemText from '@mui/material/ListItemText';
import ListItemButton from '@mui/material/ListItemButton';
import List from '@mui/material/List';
import Divider from '@mui/material/Divider';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import CloseIcon from '@mui/icons-material/Close';
import { MenuItem, Select } from '@mui/material';
import CodeContainer from './CodeContainer';
import { localContractFetch } from '../common/LocalContractFetcher';

export default function ContractTemplateWizard({ handleFileChange, handleUpload }) {
  const [open, setOpen] = React.useState(false);
  const [contract, setContract] = useState("hello_world_logging");
  const [code, setCode] = useState(localContractFetch(contract));

  const handleChange = (event) => {
    setContract(event.target.value);
    setCode(localContractFetch(event.target.value));
  }

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const handleSave = () => {
    setOpen(false);
    handleUpload(contract);
  }

  return (
    <React.Fragment>
      <Button variant="outlined" onClick={handleClickOpen} style={{ color: 'black', size: '2em' }}>
        🧙‍♂️ Open Contract Wizard 🧙‍♂️
      </Button>
      <Dialog
        fullScreen
        open={open}
        onClose={handleClose}
      >
        <AppBar sx={{ position: 'relative' }}>
          <Toolbar>
            <IconButton
              edge="start"
              color="inherit"
              onClick={handleClose}
              aria-label="close"
            >
              <CloseIcon />
            </IconButton>
            <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
              Contract Template Wizard - <i>select an example contract to upload and view.</i>
            </Typography>
            <Button autoFocus color="inherit" onClick={handleSave}>
              Upload
            </Button>
          </Toolbar>
        </AppBar>
        <Select
          labelId="demo-simple-select-label"
          id="demo-simple-select"
          value={contract}
          label="Template Contract"
          onChange={handleChange}
          style={{ fontSize: '1m', backgroundColor: 'white', margin: '2%', color: 'black' }}
        >
          <MenuItem value={"account"}>Account</MenuItem>
          {/* <MenuItem value={"alloc"}>Alloc</MenuItem> */}
          {/* <MenuItem value={"atomic_multiswap"}>Atomic Multiswap</MenuItem> */}
          <MenuItem value={"atomic_swap"}>Atomic Swap</MenuItem>
          <MenuItem value={"auth"}>Auth</MenuItem>
          <MenuItem value={"custom_types"}>Custom Types</MenuItem>
          <MenuItem value={"errors"}>Errors</MenuItem>
          {/* <MenuItem value={"eth_abi"}>ETH ABI</MenuItem> */}
          <MenuItem value={"events"}>Events</MenuItem>
          <MenuItem value={"fuzzing"}>Fuzzing</MenuItem>
          <MenuItem value={"hello_world_logging"}>Hello World Logging</MenuItem>
          <MenuItem value={"increment"}>Increment</MenuItem>
          <MenuItem value={"logging"}>Logging</MenuItem>
          {/* <MenuItem value={"mint_lock"}>Mint Lock</MenuItem> */}
          <MenuItem value={"simple_account"}>Simple Account</MenuItem>
          <MenuItem value={"single_offer"}>Single Offer</MenuItem>
          <MenuItem value={"timelock"}>Timelock</MenuItem>
          <MenuItem value={"ttl"}>TTL</MenuItem>
        </Select>
        <div style={{
          display: 'flex',
          margin: '10px',
          overflow: 'scroll',
          height: '100%',
        }}>
          <CodeContainer isWizard={true} filename={contract} originalText={code} />
        </div>
      </Dialog>
    </React.Fragment>
  );
}