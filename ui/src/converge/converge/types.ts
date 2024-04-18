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

import type { HrlB64WithContext, Hrl } from '@lightningrodlabs/we-applet';

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
 | ({ type: 'Settings'; } & Settings)
 | ({ type: 'CriterionComment'; } & CriterionComment)
 | ({ type: 'Proposal'; } & Proposal)
 | ({ type: 'Criterion'; } & Criterion)
 | ({  type: 'Deliberation'; } & Deliberation);



export interface Deliberation { 
  title: string;

  description: string;

  settings: string;

  attachments?: HrlB64WithContext[];
}

export interface Objection {
  tag: string,
}

// export interface Action {
//    author: AgentPubKey,
//   timestamp: Date,
//   action_seq: any,
//   prev_action: ActionHash,
//   base_address: ActionHash,
//   target_address: ActionHash,
//   zome_index: any,
//   link_type: any,
//   tag: String,
//   weight: any,
// }

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
  attachments: string[]
}

export interface CreateProposalInput { 
  proposal: Proposal;
  deliberation: ActionHash;
}


export interface Outcome { 
  title: string;
  description: string;
  outcome_attachment: string;
  proposal?: ActionHash;
}

export interface CreateOutcomeInput { 
  outcome: Outcome;
  deliberation: ActionHash;
}



export interface CriterionComment { 
  comment: string;

  comment_reference: ActionHash;

  objection_reference: ActionHash;

  alternative_reference: ActionHash;

  author: AgentPubKey;

  created: number;
}




export interface Settings { 
  discussion_app: string;
}

