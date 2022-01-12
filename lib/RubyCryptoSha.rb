# frozen_string_literal: true
require "rutie"
require_relative "RubyCryptoSha/version"

module RubyCryptoSha
  class Error < StandardError; end
  Rutie.new(:RubyCryptoSha).init 'Init_ruby_crypto_sha', __dir__
end
