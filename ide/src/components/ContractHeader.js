import React, { Component } from 'react'
import Box from '@mui/material/Box';

class ContractHeader extends Component {
    render() {
        console.log(this.props)
        return (
          <div style={{
              display: 'flex',
              alignItems: 'center',
              flexDirection: 'row',
              flex: '5',
              justifyContent: 'center',
              alignContent: 'center',
          }}>
            <h1>{this.props?.name ? "[Name]: " + this.props?.name : ""}</h1>
          </div>
        )
    }
}
export default ContractHeader;