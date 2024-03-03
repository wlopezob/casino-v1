
variable project_name_prefix {
    type = string
    description = "the prefix of the project name"
}

variable region {
  description = "The region to deploy the resources"
  type        = string
}

variable ami_id {
  description = "The AMI ID to use for the EC2 instances"
  type        = string
}

variable instance_type {
  description = "The instance type to use for the EC2 instances"
  type        = string
}

variable subnet_id {
  description = "The subnet ID to use for the EC2 instances"
  type        = string
}

variable key_name {
  description = "The key pair name to use for the EC2 instances"
  type        = string
}