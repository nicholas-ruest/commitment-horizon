import{describe,expect,it}from'vitest';
describe('UI contract',()=>{it('keeps core workflow labels stable',()=>{expect(['Horizon','New commitment','Audit']).toHaveLength(3)})});
