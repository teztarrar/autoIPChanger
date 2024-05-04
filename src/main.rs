use std::process::Command;
use std::io::{Result};
use std::time::Duration;

fn main() -> Result<()> {
    // Install Tor using sudo apt
    install_tor()?;
    
    // Configure system-wide proxy settings to use Tor SOCKS proxy
    configure_proxy()?;
    
    println!("Tor installation and proxy configuration completed successfully.");
    println!("Your IP should now be changed. You can test it by running 'curl ifconfig.me'.");
    
    // Display Tor circuit path when IP changes
    loop {
        // Get the HTML response from the Tor check service
        let output = Command::new("torsocks")
            .arg("curl")
            .arg("https://check.torproject.org")
            .output()?;
        
        if output.status.success() {
            // Extract the IP address from the HTML response
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                if let Some(ip) = extract_ip_address(&output_str) {
                    println!("Tor circuit path: {}", ip);
                }
            }
        }
        
        // Wait for 50 seconds before checking again
        std::thread::sleep(Duration::from_secs(50));
    }
}

fn install_tor() -> Result<()> {
    println!("Installing Tor...");
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("-y") // Automatically answer "yes" to install prompts
        .arg("tor")
        .output()?;
    
    if output.status.success() {
        println!("Tor installation completed successfully.");
    } else {
        println!("Error occurred during Tor installation:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

fn configure_proxy() -> Result<()> {
    // Set up proxy configuration
    let proxy_config = "[Proxy]\n\
                        Method=manual\n\
                        HTTPProxy=\n\
                        HTTPSProxy=\n\
                        SOCKS5Proxy=127.0.0.1:9050\n\
                        NoProxy=localhost,127.0.0.1\n";
    
    // Write proxy configuration to /etc/environment using sudo
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("echo '{}' | sudo tee /etc/environment", proxy_config))
        .output()?;
    
    if output.status.success() {
        println!("Proxy configuration set to use Tor SOCKS proxy.");
    } else {
        println!("Error occurred while setting up proxy configuration:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn extract_ip_address(html: &str) -> Option<String> {
    // Extract the IP address between the <strong> tags
    let start_tag = "<strong>";
    let end_tag = "</strong>";
    
    if let Some(start_idx) = html.find(start_tag) {
        if let Some(end_idx) = html[start_idx + start_tag.len()..].find(end_tag) {
            return Some(html[start_idx + start_tag.len()..start_idx + start_tag.len() + end_idx].to_owned());
        }
    }
    
    None
}
