# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Contract Translation Rust --> DTR --> Rust' do
  let(:base_directory) { './stellar_soroban' }
  let(:official_directory) { "#{base_directory}/official_sdf_examples" }
  let(:unofficial_directory) { "#{base_directory}/unofficial_digicus_examples" }

  context 'when official SDF Example' do
    it 'translates the hello world contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/hello_world/")
    end

    it 'translates the increment contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/increment/")
    end

    it 'translates the logging contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/logging/")
    end
  end

  context 'when unofficial Digicus Example' do
    it 'translates the answer to life contract' do
      assert_translates_rust_to_dtr_and_back("#{unofficial_directory}/answer_to_life/")
    end
  end
end
