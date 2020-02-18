import { Component, OnInit } from '@angular/core';
import { HttpClient, HttpResponse } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { Document } from 'src/app/class/document';

@Component({
  selector: 'app-document-detail',
  templateUrl: './document-detail.component.html',
  styleUrls: ['./document-detail.component.css']
})
export class DocumentDetailComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  document: Document = new Document();
  pdf: [''];
  downloadedFile: Blob = new Blob();

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.http.get<Document>("/document/" + this.id).subscribe((val) => {
      this.document = val;
    });
  }

  onFileChange(event) {
    if (event.target.files.length > 0) {
      const file = event.target.files[0];
      this.pdf = file;
    }
  }

  submit() {
    this.document.due_date = new Date(this.document.due_date + " 12:00:00");
    this.http.post<Document>("/document/" + this.id, this.document).subscribe((val) => {
      this.document = val;
      alert("Sikeres mentés!");
    });
  }

  download() {
    this.http.get("/file/" + this.id, { responseType: 'blob' }).subscribe((data) => window.open(URL.createObjectURL(data)));
  }

  remove() {
    this.http.post<Document>("/document/" + this.id + "/remove", []).subscribe((val) => this.document = val);
  }

  documentUpload() {
    this.http.post<Document>("/document/" + this.id + "/upload_file", this.pdf).subscribe((val) => {
      this.document = val;
      alert("Sikeres feltöltés");
    });
  }

}
