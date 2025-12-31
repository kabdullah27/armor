export interface KeyMetadata {
  fingerprint: string;
  key_type: 'Rsa2048' | 'Rsa4096' | 'Ed25519' | 'Curve25519';
  user_id: UserId;
  created_at: string;
  expires_at?: string;
  is_private: boolean;
  is_favorite: boolean;
  tags: string[];
}

export interface UserId {
  name: string;
  email: string;
  comment?: string;
}

export interface OperationResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface EncryptionResult {
  output_file: string;
  size: number;
  recipients: string[];
  signed: boolean;
}

export interface DecryptionResult {
  output_file: string;
  size: number;
  decrypted_with: string;
  signatures: SignatureInfo[];
}

export interface SignatureInfo {
  signer: string;
  fingerprint: string;
  created_at: string;
  valid: boolean;
}
