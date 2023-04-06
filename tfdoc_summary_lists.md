# The name of the module

Top comment prefixed by "Title: " and the following lines
will be at the top of the Markdown file

## Resources

* tests/simple/variables.tf : `aws_instance.this`: tfdoc keeps comments right on top of resource, variable and output blocks. All variables and outputs are kept. Only resources with comments on top are.
* tests/simple/variables.tf : `aws_instance.no_comment_here`: 
* tests/test-fixtures/main.tf : `aws_vpc.this`: VPC
* tests/test-fixtures/main.tf : `aws_vpc_ipv4_cidr_block_association.this`: 
* tests/test-fixtures/main.tf : `aws_vpc_dhcp_options.this`: DHCP Options Set
* tests/test-fixtures/main.tf : `aws_vpc_dhcp_options_association.this`: DHCP Options Set Association
* tests/test-fixtures/main.tf : `aws_internet_gateway.this`: Internet Gateway
* tests/test-fixtures/main.tf : `aws_route_table.public`: Publiс routes
* tests/test-fixtures/main.tf : `aws_route.public_internet_gateway`: 
* tests/test-fixtures/main.tf : `aws_route_table.private`: Private routes There are as many routing tables as the number of NAT gateways
* tests/test-fixtures/main.tf : `aws_route_table.database`: Database routes
* tests/test-fixtures/main.tf : `aws_route.database_internet_gateway`: 
* tests/test-fixtures/main.tf : `aws_route.database_nat_gateway`: 
* tests/test-fixtures/main.tf : `aws_route_table.redshift`: Redshift routes
* tests/test-fixtures/main.tf : `aws_route_table.elasticache`: Elasticache routes
* tests/test-fixtures/main.tf : `aws_route_table.intra`: Intra routes
* tests/test-fixtures/main.tf : `aws_subnet.public`: Public subnet
* tests/test-fixtures/main.tf : `aws_subnet.private`: Private subnet
* tests/test-fixtures/main.tf : `aws_subnet.database`: Database subnet
* tests/test-fixtures/main.tf : `aws_db_subnet_group.database`: Database subnet group for ${var.name}
* tests/test-fixtures/main.tf : `aws_subnet.redshift`: Redshift subnet
* tests/test-fixtures/main.tf : `aws_redshift_subnet_group.redshift`: Redshift subnet group for ${var.name}
* tests/test-fixtures/main.tf : `aws_subnet.elasticache`: ElastiCache subnet
* tests/test-fixtures/main.tf : `aws_elasticache_subnet_group.elasticache`: ElastiCache subnet group for ${var.name}
* tests/test-fixtures/main.tf : `aws_subnet.intra`: intra subnets - private subnet without NAT gateway
* tests/test-fixtures/main.tf : `aws_eip.nat`: 
* tests/test-fixtures/main.tf : `aws_nat_gateway.this`: 
* tests/test-fixtures/main.tf : `aws_route.private_nat_gateway`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.s3`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.private_s3`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.intra_s3`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.public_s3`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.dynamodb`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.private_dynamodb`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.intra_dynamodb`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_route_table_association.public_dynamodb`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ssm`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ssmmessages`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ec2`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ec2messages`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ecr_api`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.ecr_dkr`: 
* tests/test-fixtures/main.tf : `aws_vpc_endpoint.apigw`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.private`: Route table association
* tests/test-fixtures/main.tf : `aws_route_table_association.database`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.redshift`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.redshift_public`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.elasticache`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.intra`: 
* tests/test-fixtures/main.tf : `aws_route_table_association.public`: 
* tests/test-fixtures/main.tf : `aws_vpn_gateway.this`: VPN Gateway
* tests/test-fixtures/main.tf : `aws_vpn_gateway_attachment.this`: 
* tests/test-fixtures/main.tf : `aws_vpn_gateway_route_propagation.public`: 
* tests/test-fixtures/main.tf : `aws_vpn_gateway_route_propagation.private`: 
* tests/test-fixtures/main.tf : `aws_default_vpc.this`: Defaults

## Data

* tests/simple/variables.tf : `aws_ami.node`: Data blocks are not ignored
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.s3`: VPC Endpoint for S3
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.dynamodb`: VPC Endpoint for DynamoDB
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ssm`: VPC Endpoint for SSM
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ssmmessages`: VPC Endpoint for SSMMESSAGES
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ec2`: VPC Endpoint for EC2
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ec2messages`: VPC Endpoint for EC2MESSAGES
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ecr_api`: VPC Endpoint for ECR API
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.ecr_dkr`: VPC Endpoint for ECR DKR
* tests/test-fixtures/main.tf : `aws_vpc_endpoint_service.apigw`: VPC Endpoint for API Gateway

## Inputs

* `environment`: Variable descriptions will be parsed
* `create_vpc`: Controls if VPC should be created (it affects almost all resources)
* `name`: Name to be used on all the resources as identifier
* `cidr`: The CIDR block for the VPC. Default value is a valid CIDR, but not acceptable by AWS and should be overridden
* `assign_generated_ipv6_cidr_block`: Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block
* `secondary_cidr_blocks`: List of secondary CIDR blocks to associate with the VPC to extend the IP Address pool
* `instance_tenancy`: A tenancy option for instances launched into the VPC
* `public_subnet_suffix`: Suffix to append to public subnets name
* `private_subnet_suffix`: Suffix to append to private subnets name
* `intra_subnet_suffix`: Suffix to append to intra subnets name
* `database_subnet_suffix`: Suffix to append to database subnets name
* `redshift_subnet_suffix`: Suffix to append to redshift subnets name
* `elasticache_subnet_suffix`: Suffix to append to elasticache subnets name
* `public_subnets`: A list of public subnets inside the VPC
* `private_subnets`: A list of private subnets inside the VPC
* `database_subnets`: A list of database subnets
* `redshift_subnets`: A list of redshift subnets
* `elasticache_subnets`: A list of elasticache subnets
* `intra_subnets`: A list of intra subnets
* `create_database_subnet_route_table`: Controls if separate route table for database should be created
* `create_redshift_subnet_route_table`: Controls if separate route table for redshift should be created
* `enable_public_redshift`: Controls if redshift should have public routing table
* `create_elasticache_subnet_route_table`: Controls if separate route table for elasticache should be created
* `create_database_subnet_group`: Controls if database subnet group should be created
* `create_elasticache_subnet_group`: Controls if elasticache subnet group should be created
* `create_redshift_subnet_group`: Controls if redshift subnet group should be created
* `create_database_internet_gateway_route`: Controls if an internet gateway route for public database access should be created
* `create_database_nat_gateway_route`: Controls if a nat gateway route should be created to give internet access to the database subnets
* `azs`: A list of availability zones in the region
* `enable_dns_hostnames`: Should be true to enable DNS hostnames in the VPC
* `enable_dns_support`: Should be true to enable DNS support in the VPC
* `enable_nat_gateway`: Should be true if you want to provision NAT Gateways for each of your private networks
* `single_nat_gateway`: Should be true if you want to provision a single shared NAT Gateway across all of your private networks
* `one_nat_gateway_per_az`: Should be true if you want only one NAT Gateway per availability zone. Requires `var.azs` to be set, and the number of `public_subnets` created to be greater than or equal to the number of availability zones specified in `var.azs`.
* `reuse_nat_ips`: Should be true if you don't want EIPs to be created for your NAT Gateways and will instead pass them in via the 'external_nat_ip_ids' variable
* `external_nat_ip_ids`: List of EIP IDs to be assigned to the NAT Gateways (used in combination with reuse_nat_ips)
* `enable_dynamodb_endpoint`: Should be true if you want to provision a DynamoDB endpoint to the VPC
* `enable_s3_endpoint`: Should be true if you want to provision an S3 endpoint to the VPC
* `enable_ssm_endpoint`: Should be true if you want to provision an SSM endpoint to the VPC
* `ssm_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for SSM endpoint
* `ssm_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for SSM endpoint. Only a single subnet within an AZ is supported. If omitted, private subnets will be used.
* `ssm_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for SSM endpoint
* `enable_ssmmessages_endpoint`: Should be true if you want to provision a SSMMESSAGES endpoint to the VPC
* `enable_apigw_endpoint`: Should be true if you want to provision an api gateway endpoint to the VPC
* `apigw_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for API GW  endpoint
* `apigw_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for API GW endpoint
* `apigw_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for API GW endpoint. Only a single subnet within an AZ is supported. If omitted, private subnets will be used.
* `ssmmessages_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for SSMMESSAGES endpoint
* `ssmmessages_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for SSMMESSAGES endpoint. Only a single subnet within an AZ is supported. If omitted, private subnets will be used.
* `ssmmessages_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for SSMMESSAGES endpoint
* `enable_ec2_endpoint`: Should be true if you want to provision an EC2 endpoint to the VPC
* `ec2_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for EC2 endpoint
* `ec2_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for EC2 endpoint
* `ec2_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for EC2 endpoint. Only a single subnet within an AZ is supported. If omitted, private subnets will be used.
* `enable_ec2messages_endpoint`: Should be true if you want to provision an EC2MESSAGES endpoint to the VPC
* `ec2messages_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for EC2MESSAGES endpoint
* `ec2messages_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for EC2MESSAGES endpoint
* `ec2messages_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for EC2MESSAGES endpoint. Only a single subnet within an AZ is supported. If omitted, private subnets will be used.
* `enable_ecr_api_endpoint`: Should be true if you want to provision an ecr api endpoint to the VPC
* `ecr_api_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for ECR api endpoint. If omitted, private subnets will be used.
* `ecr_api_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for ECR API endpoint
* `ecr_api_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for ECR API endpoint
* `enable_ecr_dkr_endpoint`: Should be true if you want to provision an ecr dkr endpoint to the VPC
* `ecr_dkr_endpoint_subnet_ids`: The ID of one or more subnets in which to create a network interface for ECR dkr endpoint. If omitted, private subnets will be used.
* `ecr_dkr_endpoint_private_dns_enabled`: Whether or not to associate a private hosted zone with the specified VPC for ECR DKR endpoint
* `ecr_dkr_endpoint_security_group_ids`: The ID of one or more security groups to associate with the network interface for ECR DKR endpoint
* `map_public_ip_on_launch`: Should be false if you do not want to auto-assign public IP on launch
* `enable_vpn_gateway`: Should be true if you want to create a new VPN Gateway resource and attach it to the VPC
* `vpn_gateway_id`: ID of VPN Gateway to attach to the VPC
* `amazon_side_asn`: The Autonomous System Number (ASN) for the Amazon side of the gateway. By default the virtual private gateway is created with the current default Amazon ASN.
* `propagate_private_route_tables_vgw`: Should be true if you want route table propagation
* `propagate_public_route_tables_vgw`: Should be true if you want route table propagation
* `tags`: A map of tags to add to all resources
* `vpc_tags`: Additional tags for the VPC
* `igw_tags`: Additional tags for the internet gateway
* `public_subnet_tags`: Additional tags for the public subnets
* `private_subnet_tags`: Additional tags for the private subnets
* `public_route_table_tags`: Additional tags for the public route tables
* `private_route_table_tags`: Additional tags for the private route tables
* `database_route_table_tags`: Additional tags for the database route tables
* `redshift_route_table_tags`: Additional tags for the redshift route tables
* `elasticache_route_table_tags`: Additional tags for the elasticache route tables
* `intra_route_table_tags`: Additional tags for the intra route tables
* `database_subnet_tags`: Additional tags for the database subnets
* `database_subnet_group_tags`: Additional tags for the database subnet group
* `redshift_subnet_tags`: Additional tags for the redshift subnets
* `redshift_subnet_group_tags`: Additional tags for the redshift subnet group
* `elasticache_subnet_tags`: Additional tags for the elasticache subnets
* `intra_subnet_tags`: Additional tags for the intra subnets
* `dhcp_options_tags`: Additional tags for the DHCP option set
* `nat_gateway_tags`: Additional tags for the NAT gateways
* `nat_eip_tags`: Additional tags for the NAT EIP
* `vpn_gateway_tags`: Additional tags for the VPN gateway
* `enable_dhcp_options`: Should be true if you want to specify a DHCP options set with a custom domain name, DNS servers, NTP servers, netbios servers, and/or netbios server type
* `dhcp_options_domain_name`: Specifies DNS name for DHCP options set
* `dhcp_options_domain_name_servers`: Specify a list of DNS server addresses for DHCP options set, default to AWS provided
* `dhcp_options_ntp_servers`: Specify a list of NTP servers for DHCP options set
* `dhcp_options_netbios_name_servers`: Specify a list of netbios servers for DHCP options set
* `dhcp_options_netbios_node_type`: Specify netbios node_type for DHCP options set
* `manage_default_vpc`: Should be true to adopt and manage Default VPC
* `default_vpc_name`: Name to be used on the Default VPC
* `default_vpc_enable_dns_support`: Should be true to enable DNS support in the Default VPC
* `default_vpc_enable_dns_hostnames`: Should be true to enable DNS hostnames in the Default VPC
* `default_vpc_enable_classiclink`: Should be true to enable ClassicLink in the Default VPC
* `default_vpc_tags`: Additional tags for the Default VPC

## Outputs

* `name`: We can have both comments on top and within outputs and variables
* `vpc_id`: The ID of the VPC
* `vpc_cidr_block`: The CIDR block of the VPC
* `default_security_group_id`: The ID of the security group created by default on VPC creation
* `default_network_acl_id`: The ID of the default network ACL
* `default_route_table_id`: The ID of the default route table
* `vpc_instance_tenancy`: Tenancy of instances spin up within VPC
* `vpc_enable_dns_support`: Whether or not the VPC has DNS support
* `vpc_enable_dns_hostnames`: Whether or not the VPC has DNS hostname support
* `vpc_main_route_table_id`: The ID of the main route table associated with this VPC
* `vpc_secondary_cidr_blocks`: List of secondary CIDR blocks of the VPC
* `private_subnets`: List of IDs of private subnets
* `private_subnets_cidr_blocks`: List of cidr_blocks of private subnets
* `public_subnets`: List of IDs of public subnets
* `public_subnets_cidr_blocks`: List of cidr_blocks of public subnets
* `database_subnets`: List of IDs of database subnets
* `database_subnets_cidr_blocks`: List of cidr_blocks of database subnets
* `database_subnet_group`: ID of database subnet group
* `redshift_subnets`: List of IDs of redshift subnets
* `redshift_subnets_cidr_blocks`: List of cidr_blocks of redshift subnets
* `redshift_subnet_group`: ID of redshift subnet group
* `elasticache_subnets`: List of IDs of elasticache subnets
* `elasticache_subnets_cidr_blocks`: List of cidr_blocks of elasticache subnets
* `intra_subnets`: List of IDs of intra subnets
* `intra_subnets_cidr_blocks`: List of cidr_blocks of intra subnets
* `elasticache_subnet_group`: ID of elasticache subnet group
* `elasticache_subnet_group_name`: Name of elasticache subnet group
* `public_route_table_ids`: List of IDs of public route tables
* `private_route_table_ids`: List of IDs of private route tables
* `database_route_table_ids`: List of IDs of database route tables
* `redshift_route_table_ids`: List of IDs of redshift route tables
* `elasticache_route_table_ids`: List of IDs of elasticache route tables
* `intra_route_table_ids`: List of IDs of intra route tables
* `nat_ids`: List of allocation ID of Elastic IPs created for AWS NAT Gateway
* `nat_public_ips`: List of public Elastic IPs created for AWS NAT Gateway
* `natgw_ids`: List of NAT Gateway IDs
* `igw_id`: The ID of the Internet Gateway
* `vgw_id`: The ID of the VPN Gateway
* `default_vpc_id`: The ID of the VPC
* `default_vpc_cidr_block`: The CIDR block of the VPC
* `default_vpc_default_security_group_id`: The ID of the security group created by default on VPC creation
* `default_vpc_default_network_acl_id`: The ID of the default network ACL
* `default_vpc_default_route_table_id`: The ID of the default route table
* `default_vpc_instance_tenancy`: Tenancy of instances spin up within VPC
* `default_vpc_enable_dns_support`: Whether or not the VPC has DNS support
* `default_vpc_enable_dns_hostnames`: Whether or not the VPC has DNS hostname support
* `default_vpc_main_route_table_id`: The ID of the main route table associated with this VPC
* `vpc_endpoint_s3_id`: VPC Endpoints The ID of VPC endpoint for S3
* `vpc_endpoint_s3_pl_id`: The prefix list for the S3 VPC endpoint.
* `vpc_endpoint_dynamodb_id`: The ID of VPC endpoint for DynamoDB
* `vpc_endpoint_dynamodb_pl_id`: The prefix list for the DynamoDB VPC endpoint.
* `vpc_endpoint_ssm_id`: The ID of VPC endpoint for SSM
* `vpc_endpoint_ssm_network_interface_ids`: One or more network interfaces for the VPC Endpoint for SSM.
* `vpc_endpoint_ssm_dns_entry`: The DNS entries for the VPC Endpoint for SSM.
* `vpc_endpoint_ssmmessages_id`: The ID of VPC endpoint for SSMMESSAGES
* `vpc_endpoint_ssmmessages_network_interface_ids`: One or more network interfaces for the VPC Endpoint for SSMMESSAGES.
* `vpc_endpoint_ssmmessages_dns_entry`: The DNS entries for the VPC Endpoint for SSMMESSAGES.
* `vpc_endpoint_ec2_id`: The ID of VPC endpoint for EC2
* `vpc_endpoint_ec2_network_interface_ids`: One or more network interfaces for the VPC Endpoint for EC2
* `vpc_endpoint_ec2_dns_entry`: The DNS entries for the VPC Endpoint for EC2.
* `vpc_endpoint_ec2messages_id`: The ID of VPC endpoint for EC2MESSAGES
* `vpc_endpoint_ec2messages_network_interface_ids`: One or more network interfaces for the VPC Endpoint for EC2MESSAGES
* `vpc_endpoint_ec2messages_dns_entry`: The DNS entries for the VPC Endpoint for EC2MESSAGES.
* `azs`: Static values (arguments) A list of availability zones specified as argument to this module

## Files

* `tests/simple/variables.tf`
* `tests/test-fixtures/outputs.tf`
* `tests/test-fixtures/main.tf`
* `tests/test-fixtures/variables.tf`
