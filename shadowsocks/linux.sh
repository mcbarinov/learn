#!/bin/bash

############################### Update packages ################################
apt update && apt upgrade -y

########################## Install Midnight Commander ##########################
apt install mc zip

############################ SSH: disable password #############################
nano /etc/ssh/sshd_config

: '
PermitRootLogin prohibit-password
'

############################### SSH: change port ###############################
mkdir -p /etc/systemd/system/ssh.socket.d

cat >/etc/systemd/system/ssh.socket.d/listen.conf <<EOF
[Socket]
ListenStream=
ListenStream=1234
EOF

systemctl daemon-reload
systemctl restart ssh.socket
systemctl status ssh

################################# Block pings ##################################

echo 'net.ipv4.icmp_echo_ignore_all=1' | tee -a /etc/sysctl.conf
echo 'net.core.default_qdisc=fq' | tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_congestion_control=bbr' | tee -a /etc/sysctl.conf
sysctl -p
sysctl -a | grep congestion

########################## Download shadowsocks-rust ###########################
cd /usr/local/bin

wget https://github.com/shadowsocks/shadowsocks-rust/releases/download/v1.20.3/shadowsocks-v1.20.3.x86_64-unknown-linux-gnu.tar.xz -O shadowsocks.tar.xz
tar -xvf shadowsocks.tar.xz && rm shadowsocks.tar.xz

######################## Create shadowsocks-rust config ########################

mkdir /etc/shadowsocks

pass=`ssservice genkey -m "2022-blake3-chacha20-poly1305"`
cat > /etc/shadowsocks/config.json << EOF
{
    "servers": [
        {
            "server": "::",
            "server_port": 1234,
            "password": "$pass",
            "timeout": 300,
            "method": "2022-blake3-chacha20-poly1305",
            "mode": "tcp_and_udp",
            "fast_open": false
        }
    ]
}
EOF
unset pass

############################ Create Systemd service ############################

cat > /etc/systemd/system/ssserver.service << EOF
[Unit]
Description=shadowsocks-rust service
After=network.target

[Service]
ExecStart=/usr/local/bin/ssserver --log-without-time -c /etc/shadowsocks/config.json
Restart=always
RestartSec=10
Type=simple
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
AmbientCapabilities=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
EOF

############################ Start shadowsocks-rust ############################
systemctl enable ssserver
systemctl start ssserver

################################### Copy url ###################################
ssurl --encode /etc/shadowsocks/config.json

#################################### Clear #####################################
history -c && history -w