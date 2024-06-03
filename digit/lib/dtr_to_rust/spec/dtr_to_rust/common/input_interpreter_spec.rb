# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Common::InputInterpreter do
  describe '#interpret' do
    it 'detects a simple variable' do
      expect(described_class.interpret('foo')).to eq({
                                                       value: 'foo',
                                                       type: 'variable',
                                                       needs_reference: true
                                                     })
    end

    it 'detects a more complex variable' do
      expect(described_class.interpret('FOO_BAR_10')).to eq({
                                                              value: 'FOO_BAR_10',
                                                              type: 'variable',
                                                              needs_reference: true
                                                            })
    end

    it 'detects a simple string' do
      expect(described_class.interpret('"foo"')).to eq({
                                                         value: '"foo"',
                                                         type: 'string',
                                                         needs_reference: false
                                                       })
    end

    it 'detects a more complex string' do
      expect(described_class.interpret('"foo bar baz on the faz"')).to eq({
                                                                            value: '"foo bar baz on the faz"',
                                                                            type: 'string',
                                                                            needs_reference: false
                                                                          })
    end

    it 'detects number' do
      expect(described_class.interpret(1)).to eq({
                                                   value: 1,
                                                   type: 'number',
                                                   needs_reference: false
                                                 })
    end
  end
end
