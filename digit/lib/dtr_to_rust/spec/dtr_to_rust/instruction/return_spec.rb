# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Return do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = DTRCore::Instruction.new('Return', ['foo'], nil, 0)
      expect(described_class.handle(instruction, 0, [], [], false)).to eq('        foo')
    end
  end
end
