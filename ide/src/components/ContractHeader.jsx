import React, { Component } from 'react'

import '.././styles/ContractHeader.css';

class ContractHeader extends Component {
    render() {
        return (
          <div className='contract-header'>
            <h1>{this.props?.name ? "[Name]: " + this.props?.name : "Upload a Contract ➡️"}</h1>
          </div>
        )
    }
}
export default ContractHeader;