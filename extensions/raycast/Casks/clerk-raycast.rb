cask "clerk-raycast" do
  version "0.1.0"
  sha256 "71b6b06f84905037b9cce67d59c87bd9f016591613736e4e6351e3421aec3c68"

  url "https://github.com/Sawmills/clerk-cli/releases/download/raycast-v#{version}/clerk-raycast-#{version}.tar.gz"
  name "Clerk Admin for Raycast"
  desc "Manage Clerk users and organizations from Raycast"
  homepage "https://github.com/Sawmills/clerk-cli"

  depends_on macos: ">= :monterey"
  depends_on cask: "raycast"

  raycast_extensions_dir = "#{Dir.home}/Library/Application Support/com.raycast.macos/extensions"
  extension_name = "clerk-admin"
  install_dir = "#{raycast_extensions_dir}/#{extension_name}"

  preflight do
    system_command "/bin/mkdir",
                   args: ["-p", raycast_extensions_dir]
  end

  pkg_dir = staged_path.join("clerk-raycast-#{version}")

  postflight do
    system_command "/bin/mkdir",
                   args: ["-p", install_dir]
    
    system_command "/bin/cp",
                   args: ["-r", "#{pkg_dir}/src", install_dir]
    system_command "/bin/cp",
                   args: ["-r", "#{pkg_dir}/assets", install_dir]
    system_command "/bin/cp",
                   args: ["#{pkg_dir}/package.json", install_dir]
    system_command "/bin/cp",
                   args: ["#{pkg_dir}/tsconfig.json", install_dir]
    
    Dir.chdir(install_dir) do
      system_command "/usr/local/bin/npm",
                     args: ["install", "--production"],
                     print_stdout: true
    end

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
  end

  uninstall delete: install_dir

  zap trash: install_dir
end
