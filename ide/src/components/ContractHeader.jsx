import React, { Component } from 'react'

import '.././styles/ContractHeader.css';
import EditableTitle from './EditableTitle';

class ContractHeader extends Component {
  render() {
    return (
      <div className='contract-header'>
        {
          this.props?.name ?
            <h1>{this.props.name}</h1>
            : <h1>"Upload a Contract ➡️"</h1>
        }
      </div>
    )
  }
}
export default ContractHeader;