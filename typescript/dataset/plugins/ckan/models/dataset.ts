import { z } from "zod"
import { CkanOrganization } from "./organization.ts"
import { CkanResource } from "./resource.ts"
import { CkanTag } from "./tag.ts"

export const CkanDataset = z
  .object({
    resources: z.array(CkanResource).describe("List of resources"),
    organization: CkanOrganization.optional().describe("Organization information"),
    tags: z.array(CkanTag).describe("List of tags"),
    id: z.string().describe("Package identifier"),
    name: z.string().describe("Package name"),
    title: z.string().optional().describe("Package title"),
    notes: z.string().optional().describe("Package notes/description"),
    version: z.string().optional().describe("Package version"),
    license_id: z.string().optional().describe("License identifier"),
    license_title: z.string().optional().describe("License title"),
    license_url: z.string().optional().describe("License URL"),
    author: z.string().optional().describe("Package author"),
    author_email: z.string().optional().describe("Package author email"),
    maintainer: z.string().optional().describe("Package maintainer"),
    maintainer_email: z.string().optional().describe("Package maintainer email"),
    metadata_created: z.string().optional().describe("Metadata creation timestamp"),
    metadata_modified: z.string().optional().describe("Metadata modification timestamp"),
  })
  .describe("CKAN Package interface")

export type CkanDataset = z.infer<typeof CkanDataset>
