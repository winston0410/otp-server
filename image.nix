{ pkgs, package, ... }:
pkgs.dockerTools.buildLayeredImage {
  name = "otp-server";
  tag = "latest";
  config = { Cmd = [ "${package}/bin/${package.pname}" ]; };
}
