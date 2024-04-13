import React, { Component } from 'react'
import Box from '@mui/material/Box';


class ContractContainer extends Component {
    render() {
        return (
          <div style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              flexDirection: 'column',
              flex: '1',
              backgroundColor: 'rgb(39 207 230)',
              width: '100%',
              height: '100%'
          }}>
            <Box
                style={{
                  backgroundColor: 'white',
                  color: 'black',
                }}
                height={200}
                width={200}
                my={4}
                display="flex"
                alignItems="center"
                gap={4}
                p={2}
                sx={{ border: '2px solid grey' }}
            >
              The contract
            </Box>
          </div>
        
        )
    }
}
export default ContractContainer;