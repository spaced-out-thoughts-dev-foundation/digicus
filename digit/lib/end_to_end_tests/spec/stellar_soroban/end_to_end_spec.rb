# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Contract Translation Rust --> DTR --> Rust' do
  let(:base_directory) { './stellar_soroban' }
  let(:official_directory) { "#{base_directory}/official_sdf_examples" }
  let(:unofficial_directory) { "#{base_directory}/unofficial_digicus_examples" }

  context 'when official SDF Example' do
    it 'translates the account contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/account/")
    end

    it 'translates the alloc contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/alloc/")
    end

    it 'translates the atomic_multiswap contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/atomic_multiswap/")
    end

    it 'translates the atomic_swap contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/atomic_swap/")
    end

    it 'translates the auth contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/auth/")
    end

    it 'translates the custom_types contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/custom_types/")
    end

    it 'translates the errors contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/errors/")
    end

    it 'translates the eth_abi contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/eth_abi/")
    end

    it 'translates the events contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/events/")
    end

    it 'translates the hello world contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/hello_world/")
    end

    it 'translates the increment contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/increment/")
    end

    it 'translates the logging contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/logging/")
    end

    it 'translates the mint_lock contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/mint_lock/")
    end

    it 'translates the simple_account contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/simple_account/")
    end

    it 'translates the single_offer contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/single_offer/")
    end

    it 'translates the timelock contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/timelock/")
    end

    it 'translates the ttl contract' do
      assert_translates_rust_to_dtr_and_back("#{official_directory}/ttl/")
    end
  end

  context 'when unofficial Digicus Example' do
    it 'translates the answer to life contract' do
      assert_translates_rust_to_dtr_and_back("#{unofficial_directory}/answer_to_life/")
    end
  end
end
