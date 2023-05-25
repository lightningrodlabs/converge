import type { 
  Record, 
  ActionHash,
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type ConvergeSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'Proposal'; } & Proposal)
 | ({ type: 'Criterion'; } & Criterion)
 | ({  type: 'Deliberation'; } & Deliberation);



export interface Deliberation { 
  title: string;

  description: string;

  settings: string;
}




export interface Criterion { 
  title: string;
}

export interface CreateCriterionInput {
  criterion: Criterion;
  deliberation: ActionHash;
}




export interface Proposal { 
  title: string;

  description: string;
}

export interface CreateProposalInput { 
  proposal: Proposal;
  deliberation: ActionHash;
}
