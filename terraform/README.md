# Terraform
### start terraform
terraform init

### update providers
terraform init -upgrade

### create the plan
terraform plan -out main.tfplan

### exeute the plan
terraform apply "main.tfplan"

### create the plan destroy
terraform plan -destroy -out main.destroy.tfplan

### execute the plan destroy
terraform apply main.destroy.tfplan


