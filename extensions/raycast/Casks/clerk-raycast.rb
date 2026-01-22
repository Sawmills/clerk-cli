cask "clerk-raycast" do
  version "0.2.2"
  sha256 "ce25af47a2ffc9c2849133df6abd9e7819aeaf213e9249741845263e990a5c12"

  url "https://github.com/Sawmills/clerk-cli/releases/download/raycast-v#{version}/clerk-raycast-#{version}.tar.gz"
  name "Clerk Admin for Raycast"
  desc "Manage Clerk users and organizations from Raycast"
  homepage "https://github.com/Sawmills/clerk-cli"

  depends_on macos: ">= :monterey"
  depends_on cask: "raycast"

  postflight do
    raycast_extensions_dir = "#{Dir.home}/Library/Application Support/com.raycast.macos/extensions"
    extension_name = "clerk-admin"
    install_dir = "#{raycast_extensions_dir}/#{extension_name}"
    pkg_dir = "#{staged_path}/clerk-raycast-#{version}"

    system_command "/bin/mkdir", args: ["-p", raycast_extensions_dir]
    system_command "/bin/rm", args: ["-rf", install_dir]
    system_command "/bin/cp", args: ["-r", pkg_dir, install_dir]

    puts "\n✅ Clerk Admin extension installed to Raycast!"
    puts "\nTo use:"
    puts "  1. Restart Raycast or press ⌘ R to reload"
    puts "  2. Open Raycast (⌘ Space)"
    puts "  3. Search for 'Clerk Admin'"
    puts "  4. Configure your API key in preferences (⌘ ,)"
    puts "\nAvailable commands:"
    puts "  - Search Organizations"
    puts "  - Search Users"
    puts "  - Impersonate User"
    puts "  - Generate JWT"
    puts "  - Organization Members"
    puts "  - Switch Instance"
  end

  uninstall delete: "#{Dir.home}/Library/Application Support/com.raycast.macos/extensions/clerk-admin"

  zap trash: "#{Dir.home}/Library/Application Support/com.raycast.macos/extensions/clerk-admin"
end
