import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { FolderNewComponent } from './folder-new.component';

describe('FolderNewComponent', () => {
  let component: FolderNewComponent;
  let fixture: ComponentFixture<FolderNewComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ FolderNewComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(FolderNewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
