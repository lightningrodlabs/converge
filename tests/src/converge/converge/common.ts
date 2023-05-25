import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleDeliberation(cell: CallableCell, partialDeliberation = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  settings: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialDeliberation
    };
}

export async function createDeliberation(cell: CallableCell, deliberation = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_deliberation",
      payload: deliberation || await sampleDeliberation(cell),
    });
}



export async function sampleCriterion(cell: CallableCell, partialCriterion = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialCriterion
    };
}

export async function createCriterion(cell: CallableCell, criterion = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_criterion",
      payload: criterion || await sampleCriterion(cell),
    });
}



export async function sampleProposal(cell: CallableCell, partialProposal = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialProposal
    };
}

export async function createProposal(cell: CallableCell, proposal = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_proposal",
      payload: proposal || await sampleProposal(cell),
    });
}

