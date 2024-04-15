import React, { Component } from 'react'
import Box from '@mui/material/Box';

class ContractHeader extends Component {
    render() {
        return (
          <div style={{
              display: 'flex',
              alignItems: 'center',
              flexDirection: 'row',
              flex: '1',
              justifyContent: 'center',
              alignContent: 'center',
          }}>
            <h1>Contract Header</h1>
          </div>
        )
    }
}
export default ContractHeader;