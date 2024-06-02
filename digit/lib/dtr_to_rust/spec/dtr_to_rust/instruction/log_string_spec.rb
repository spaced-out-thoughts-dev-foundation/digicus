# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::LogString do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = {
        instruction: 'log_string',
        inputs: ['env', '"count: {}"', 'count'],
        assign: nil,
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('log!(env,"count: {}",count);')
    end
  end
end
