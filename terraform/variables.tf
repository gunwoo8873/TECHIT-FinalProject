variable "web_port" {
  description = "The Port the server sill use for HTTP Request"
  default = 8000
}

output "public_ip" {
  value = aws_instance.simple.public_ip
}