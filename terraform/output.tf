output ip_address {
  value       = "ssh -i tallerssh.pem ec2-user@${aws_instance.casino_ec2.public_ip}"
}
