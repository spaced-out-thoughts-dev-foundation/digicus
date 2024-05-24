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
              backgroundColor: 'rgb(39 190 230)',
              margin: '0em 2em',
              borderRadius: '10px',
              padding: '1em',
              fontSize: '1em',
              boxShadow: '5px 5px 5px black',
          }}>
            <h1>{this.props?.name ? "[Name]: " + this.props?.name : "Upload a Contract -->"}</h1>
          </div>
        )
    }
}
export default ContractHeader;