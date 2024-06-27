# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Subtract do
  describe '#handle' do
    it 'returns the correct Rust code when not seen assign var before' do
      instruction = DTRCore::Instruction.new('subtract', %w[foo bar], 'foo', 0)
      expect(described_class.handle(instruction, 0, [], [], false, {}, {})).to eq('        let mut foo = foo - bar;')
    end
  end
end
