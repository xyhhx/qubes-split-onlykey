# # SPLIT SSH CONFIGURATION >>>
SSH_VAULT_VM="sys-onlykey"

if [ "$SSH_VAULT_VM" != "" ]; then
  export SSH_AUTH_SOCK="/home/user/.SSH_AGENT_$SSH_VAULT_VM"
fi
# <<< SPLIT SSH CONFIGURATION

